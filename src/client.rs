use aws_sdk_lambda::types::FunctionConfiguration;

#[derive(Debug)]
pub struct EniAssociationsClient {
    pub ec2_client: aws_sdk_ec2::Client,
    pub eni: String,
}

impl EniAssociationsClient {
    pub async fn new(eni: String) -> EniAssociationsClient {
        let shared_config = aws_config::load_from_env().await;
        let ec2_client = aws_sdk_ec2::Client::new(&shared_config);

        Self { ec2_client, eni }
    }

    pub async fn list_eni_associations(&self) -> Result<(), anyhow::Error> {
        let shared_config = aws_config::load_from_env().await;
        let lambda_client = aws_sdk_lambda::Client::new(&shared_config);

        let res = self
            .ec2_client
            .describe_network_interfaces()
            .network_interface_ids(&self.eni)
            .send()
            .await?;

        let (subnet_id, security_groups) = match res.network_interfaces() {
            Some(network_interface) => (
                network_interface[0].subnet_id().unwrap(),
                network_interface[0].groups().unwrap(),
            ),
            None => panic!("Can't find network interface"),
        };

        println!(
            "Found {} with {} using SecurityGroups {}",
            self.eni,
            subnet_id,
            security_groups
                .iter()
                .map(|s| s.group_id().unwrap())
                .collect::<Vec<_>>()
                .join(", ")
        );

        let mut lambda_functions: Vec<FunctionConfiguration> = Vec::new();
        let mut res = lambda_client
            .list_functions()
            .function_version(aws_sdk_lambda::types::FunctionVersion::All)
            .send()
            .await?;
        lambda_functions.append(&mut res.functions().unwrap().to_vec());

        // paginate
        while res.next_marker.is_some() {
            res = lambda_client
                .list_functions()
                .function_version(aws_sdk_lambda::types::FunctionVersion::All)
                .marker(res.next_marker.clone().unwrap())
                .send()
                .await?;

            lambda_functions.append(&mut res.functions().unwrap().to_vec());
        }

        // filter out lambda functions WITHOUT:
        // - VPC
        // - subnetIds
        // - securityGroupIds containing ENI securityGroupId
        lambda_functions = lambda_functions
            .iter()
            .filter(|s| s.vpc_config().is_some())
            .filter(|s| !s.vpc_config().unwrap().subnet_ids().unwrap().is_empty())
            .filter(|s| {
                s.vpc_config()
                    .unwrap()
                    .subnet_ids()
                    .unwrap()
                    .contains(&subnet_id.into())
            })
            .filter(|s| {
                s.vpc_config()
                    .unwrap()
                    .security_group_ids()
                    .unwrap()
                    .contains(&security_groups[0].group_id().unwrap().to_string())
            })
            .cloned()
            .collect::<Vec<FunctionConfiguration>>();

        if lambda_functions.is_empty() {
            println!("No Lambda functions or versions found that were using the same subnet as this ENI.");

            return Ok(());
        }

        println!(
            "Functions associated with the same subnet and security groups as {}:",
            self.eni
        );
        for lambda in lambda_functions {
            println!("\t✔️ {}", lambda.function_name().unwrap());
        }

        Ok(())
    }
}
