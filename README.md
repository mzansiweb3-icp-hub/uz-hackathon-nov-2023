# royaltiesCalculator
This project is a sophisticated royalties calculator designed to streamline and automate the distribution of royalties within the music industry. It caters to three key user roles: the artist, music user (Radio Stations), and the manager overseeing the distribution process.

Key Features:

    #1.Artist Module:
        Allows artists to input details about their songs, including metadata and ownership information.

    #2.Music User Module:
        Enables music users to submit logsheets detailing the music they played and specify the radio stations involved.

    #3.Matching:
        The system intelligently matches submitted logsheets against stored songs, automating the process of associating plays with respective tracks.

    #4.Manager Module:
        Empowers managers to enter the total amount to be distributed among artists and oversee the distribution process.

    #6.Distribution Algorithm:
        Utilizes a robust algorithm to distribute the allocated amount based on play counts, ensuring fair and accurate compensation for artists.

#Industry Relevance:

This web3 application addresses a critical need within the music industry, particularly for associations such as SAMRO in South Africa, ZIMURA in Zimbabwe, and other Collective Management Organizations (CMOs) worldwide. By automating the royalties distribution process, it enhances efficiency, reduces errors, and aligns with global standards for collecting and distributing royalties.

Welcome to your first Azle project! This example project will help you to deploy your first canister (application) to the Internet Computer (IC) decentralized cloud. It is a simple getter/setter canister. You can always refer to [The Azle Book](https://demergent-labs.github.io/azle/) for more in-depth documentation.

`dfx` is the tool you will use to interact with the IC locally and on mainnet. If you don't already have it installed:

```bash
npm run dfx_install
```

Next you will want to start a replica, which is a local instance of the IC that you can deploy your canisters to:

```bash
npm run replica_start
```

If you ever want to stop the replica:

```bash
npm run replica_stop
```

Now you can deploy your canister locally:

```bash
npm install
npm run canister_deploy_local
```

To call the methods on your canister:

```bash
npm run canister_call_get_message
npm run canister_call_set_message
```

If you run the above commands and then call `npm run canister_call_get_message` you should see:

```bash
("Hello world!")
```

Assuming you have [created a cycles wallet](https://internetcomputer.org/docs/current/developer-docs/quickstart/network-quickstart) and funded it with cycles, you can deploy to mainnet like this:

```bash
npm run canister_deploy_mainnet
```
