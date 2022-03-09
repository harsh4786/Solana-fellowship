import { useState } from "react";

import logo from "./logo.svg";
import "./App.css";
import {
    Connection,
    clusterApiUrl,
    PublicKey,
    LAMPORTS_PER_SOL,
  } from "@solana/web3.js";
  
  import {
    Keypair,
    Transaction,
    sendAndConfirmTransaction,
  } from "@solana/web3.js";
  import { TOKEN_PROGRAM_ID } from "@solana/spl-token";
  import * as Token from "@solana/spl-token";

  function App() {
    const [walletConnected, setWalletConnected] = useState(false);
    const [provider, setProvider] = useState();
    const [loading, setLoading] = useState();
  
    const [isTokenCreated, setIsTokenCreated] = useState(false);
    const [createdTokenPublicKey, setCreatedTokenPublicKey] = useState(null);
    const [mintingWalletSecretKey, setMintingWalletSecretKey] = useState(null);
    const [supplyCapped, setSupplyCapped] = useState(false);
  
    const getProvider = async () => {
      if ("solana" in window) {
        const provider = window.solana;
        if (provider.isPhantom) {
          return provider;
        }
      } else {
        window.open("https://www.phantom.app/", "_blank");
      }
    };
  
    const walletConnectionHelper = async () => {
      if (walletConnected) {
        //Disconnect Wallet
        setProvider();
        setWalletConnected(false);
      } else {
        const userWallet = await getProvider();
        if (userWallet) {
          await userWallet.connect();
          userWallet.on("connect", async () => {
            setProvider(userWallet);
            setWalletConnected(true);
          });
        }
      }
    };
    const airDropHelper = async () => {
        try {
          setLoading(true);
          const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
          const fromAirDropSignature = await connection.requestAirdrop(
            new PublicKey(provider.publicKey),
            LAMPORTS_PER_SOL
          );
          await connection.confirmTransaction(fromAirDropSignature, {
            commitment: "confirmed",
          });
    
          console.log(
            `1 SOL airdropped to your wallet ${provider.publicKey.toString()} successfully`
          );
          setLoading(false);
        } catch (err) {
          console.log(err);
          setLoading(false);
        }
      };
      const capSupplyHelper = async () => {
        try {
          setLoading(true);
          const connection = new Connection(clusterApiUrl("devnet"), "confirmed");
    
          const createMintingWallet = await Keypair.fromSecretKey(
            Uint8Array.from(Object.values(JSON.parse(mintingWalletSecretKey)))
          );
          const fromAirDropSignature = await connection.requestAirdrop(
            createMintingWallet.publicKey,
            LAMPORTS_PER_SOL
          );
          await connection.confirmTransaction(fromAirDropSignature);
    
          const creatorToken = new Token(
            connection,
            createdTokenPublicKey,
            TOKEN_PROGRAM_ID,
            createMintingWallet
          );
          await creatorToken.setAuthority(
            createdTokenPublicKey,
            null,
            "MintTokens",
            createMintingWallet.publicKey,
            [createMintingWallet]
          );
    
          setSupplyCapped(true);
          setLoading(false);
        } catch (err) {
          console.log(err);
          setLoading(false);
        }
      };
      return (
        <div className="App">
          <header className="App-header">
            {walletConnected ? (
              <p>
                <strong>Public Key:</strong> {provider.publicKey.toString()}
              </p>
            ) : (
              <p></p>
            )}
            <button onClick={walletConnectionHelper} disabled={loading}>
              {!walletConnected ? "Connect Wallet" : "Disconnect Wallet"}
            </button>
            {walletConnected ? (
              <p>
                Airdrop 1 SOL into your wallet
                <button disabled={loading} onClick={airDropHelper}>
                  AirDrop SOL{" "}
                </button>
              </p>
            ) : (
              <></>
            )}
            {walletConnected ? (
              <p>
                Create your own token
                <button disabled={loading} onClick={initialMintHelper}>
                  Initial Mint{" "}
                </button>
              </p>
            ) : (
              <></>
            )}
            <li>
              Mint More 100 tokens:{" "}
              <button disabled={loading || supplyCapped} onClick={mintAgainHelper}>
                Mint Again
              </button>
            </li>
          </header>
        </div>
      );
    }
    
    export default App;      


