# coin-mixer-challenge

## Usage

```sh
$ cargo run -- --api-base-url https://jobcoin.gemini.com/marmalade-manual/api
Given API URL:
https://jobcoin.gemini.com/marmalade-manual/api
address1
address2
address3
Read the following withdrawal addresses:
address1
address2
address3
Generated deposit address:
8mqg2VdRdPuQuXhpEGqwVPijL8xNe
Detected deposit of amount: 50
Sending to house address: og1VoyIBHRyloruErh2KG5GGzsigSRMIAP
Waiting for 64 ms
Sending 42.9005310349992900 to address2
Waiting for 69 ms
Sending 2.30478672252172512889111646891400 to address1
Waiting for 52 ms
Sending 4.49577960637641774732282633677414 to address2
Waiting for 52 ms
Sending 0.1511553952308320470152802225779 to address1
Waiting for 12 ms
Sending 0.015871428797414607798214123572 to address1
Waiting for 64 ms
Sending 0.0061567748410577355013895108867 to address1
Waiting for 72 ms
Sending 0.008203516131774903875081220510 to address2
Waiting for 91 ms
Sending 0.051926466931706116837710640272 to address3
Waiting for 67 ms
Sending 0.023665402510652294444730942853 to address1
Waiting for 2 ms
Sending 0.041923651659129418313650533637 to address2
Done!
```

## Assumptions

1. Only one user mixes their coins at a time.
2. The transfers to the withdrawal address are to be made within a
   second of detecting a deposit.
3. Coins created from scratch are also considered a deposit.
4. No fee is charged for mixing the coins.

## Design considerations

1. I aimed to deliver a PoC that teases the possibilities, so I took a
   few shortcuts.
2. The generated addresses are always checked to be unused and are
   between 26 and 35 characters long.

## Remaining work

1. Accept multiple addresses, say, from a file. That would give the
   mixer a chance to intersperse transactions belonging to different
   users.
1. Make the range of lengths of the generated addressses configurable.
1. Currently, the program fails with an error if there's a trailing
   slash in the API base URL. We should be able to handle that
   gracefully.
1. Implement graceful shutdown. Right now, upon issuing Ctrl+C, the
   program abandons whatever it's doing immediately.
1. Fake the API endpoint, for example, by using a trait that would let
   us implement testing handlers. Right now, one of the tests makes
   API calls to the real thing.
1. Come up with a measure for the effectiveness of the mixing and
   write benchmark tests that let us keep track of how our changes
   affect the effectiveness of the mixer.
1. Add persistence so that the program can resume where it left off
   and also remember the deposit addresses configured before.
1. Possibly split the program into a client and a server so that
    mutiple users can interact with it,
1. Use a proper logging library instead of using `println!()`.
