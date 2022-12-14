# How to write smart contracts?


There are multiple smart contracts authoring tools including Visual
Studio. However, the easiest and fastest way to develop smart
contracts is to use a browser-based tool known as Remix. Remix is
available on http://remix.ethereum.org. Remix is a new name and was
earlier known as browser-solidity. Remix provides a rich
integrated development environment in a browser for authoring,
developing, deploying, and troubleshooting contracts written using
the Solidity language. All contract management related activities
such as authoring, deploying, and troubleshooting can be
performed from the same environment without moving to other
windows or tabs.


Not everyone is comfortable using the online version of Remix to
author their smart contracts. Remix is an open source tool that can
be downloaded from https://github.com/ethereum/browser-Solidity and
compiled to run a private version on a local computer. Another
advantage of running Remix locally is that it can connect to local
private chain networks directly; otherwise, users will first have to
author the contract online and then copy the same to a file, compile,
and deploy manually to a private network. Let's explore Remix by
performing the following steps:


1. Navigate to remix.ethereum.org and the site will open in a
browser with a default contract as shown in the following
screenshot. If you do not need this contract, it can be
deleted


2. The first thing we need to do is to create a new contract by
selecting + from Remix's left menu bar.
3. Then, provide a name for a new Solidity file that has an
extension .sol. Name the contract HelloWorld and click on OK
to continue as shown in the following screenshot. This will
create a blank contract:


4. Type the following code in the empty authoring pane to
create your first contract. This contract will be explained in
detail in Chapter 3, Introducing Solidity. For now, it is
sufficient to understand that the contract is created using
the contract keyword; you can declare global state variables
and functions; and contracts are saved with the .sol file
extension. In the following code snippet, the HelloWorld
contracts returns the HelloWorld string when the GetHelloWorld
function is called:


Look at the action window to the right of Remix. It has got
several tabs—Compile, Run, Settings, Debugger, Analysis,
and Support. These action tabs help in compiling, deploying,
troubleshooting, and invoking contracts. The Compile tab
compiles the contract into bytecode—code that is understood
by Ethereum. It displays warnings and errors as you author
and edit the contract. These warnings and errors are to be
taken seriously and they really help in creating robust
contracts. The Run tab is the place where you will spend the
most time, apart from writing the contract. Remix comes
bundled with the Ethereum runtime within the browser. The
Run tab allows you to deploy the contract to this runtime
using the JavaScript VM environment in
the Environment option. The Injected Web3 environment is
used along with tools such as Mist and MetaMask, which will
be covered in the next chapter, and Web3 Provider can be
used when using Remix in a local environment connecting to
private network. In our case for this chapter, the default,
JavaScript VM is sufficient. The rest of the options will be
discussed later in Chapter 3, Introducing Solidity.


5. However, the important action is deployment of a contract
and that can be done using the Create button to deploy the
contract that is shown in the following screenshot:


6. Click on the Create button to deploy the contract to the
browser Ethereum runtime and this will list all the functions
available within the contract below the Create button. Since
we only had a single function GetHelloWorld, the same is
displayed as shown in the following screenshot:


. Click on the GetHelloWorld button to invoke and execute
the function. The lower pane of Remix will show the results
of execution as shown in the following screenshot:


Congratulations, you have created, deployed, and also executed a
function on your first contract. The code for the HelloWorld contract is
accompanied with this chapter and can be used in Remix if you are
not interested in typing the contract.

