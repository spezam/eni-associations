# ENI Lambda associations

```sh
💀 cargo b --release

💀 target/release/eni-associations --eni eni-02fc82fd5debd2686
Found eni-02fc82fd5debd2686 with subnet-82b0bff5 using SecurityGroups sg-9b3711ff
Functions associated with the same subnet and security groups as eni-02fc82fd5debd2686.
        ✔️ eventbridge-eta
```

#### Binary release on Mac
In order to run binary release on Mac, when 'cannot be opened because the developer cannot be verified' appears:
- open the `Security & Privacy` control panel from System Preferences
- under Security find `"eni-associations" was blocked from use because it is not from an identified developer`
- click `Allow Anyway`
- run the binary again
