import fs from "fs";
import Arweave from 'arweave';
import { actions, utils, programs, NodeWallet, MetaDataData, Metadata, updateMetadata} from '@metaplex/js'; 
import { clusterApiUrl, Connection, Keypair, LAMPORTS_PER_SOL, PublicKey, sendAndConfirmTransaction } from '@solana/web3.js';
import { scheduleJob } from 'node-schedule';
(async () => {

    const arweave = Arweave.init({
        host: 'arweave.net',
        port: 443,
        protocol: 'https',
        timeout: 20000,
        logging: false,
    });

    // Upload image to Arweave
    const data = fs.readFileSync('./home/bitcoin.png');
    
    const transaction = await arweave.createTransaction({
        data: data
    });
    
    transaction.addTag('Content-Type', 'image/png');
    
    const wallet = await arweave.wallets.getWalletFromFile('wallet.json');
    await arweave.transactions.sign(transaction, wallet);
    
    const response = await arweave.transactions.post(transaction);
    console.log(response);

    const { id } = response;
    const imageUrl = id ? `https://arweave.net/${id}` : undefined;

    // Upload metadata to Arweave

    const metadata = {
        name: "Custom NFT #1",
        symbol: "CNFT",
        description:
          "A description about my custom NFT #1",
        seller_fee_basis_points: 500,
        external_url: "",
        attributes: [
            {
                trait_type: "NFT type",
                value: "Custom"
            }
        ],
        collection: {
          name: "Test Collection",
          family: "Custom NFTs",
        },
        properties: {
          files: [
            {
              uri: imageUrl,
              type: "image/png",
            },
          ],
          category: "image",
          maxSupply: 0,
          creators: [
            {
              address: "AjAfzm1ZRcqxa4NXw3aKzvbn1PT7RbdPLfqxxQ4FmcbF",
              share: 100,
            },
          ],
        },
        image: imageUrl,
      }

    const metadataRequest = JSON.stringify(metadata);
    
    const metadataTransaction = await arweave.createTransaction({
        data: metadataRequest
    });
    
    metadataTransaction.addTag('Content-Type', 'application/json');
    
    await arweave.transactions.sign(metadataTransaction, wallet);
    
    await arweave.transactions.post(metadataTransaction);    

    //Now we mint the NFT
    const connection = new Connection(
        clusterApiUrl('devnet'),
        'confirmed',
      );
      const keypair = Keypair.generate();
      const feePayerAirdropSignature = await connection.requestAirdrop(keypair.publicKey, LAMPORTS_PER_SOL);
      await connection.confirmTransaction(feePayerAirdropSignature);
    
      const mintNFTResponse = await actions.mintNFT({
        connection,
        wallet: new NodeWallet(keypair),
        uri: 'https://34c7ef24f4v2aejh75xhxy5z6ars4xv47gpsdrei6fiowptk2nqq.arweave.net/3wXyF1wvK6ARJ_9ue-O58CMuXrz5nyHEiPFQ6z5q02E',
        maxSupply: 1
      });
    
      console.log(mintNFTResponse);


      //updating the metadata
      const MintAccount = new PublicKey("Cash8aWdyarXMGrJjaURFFkZ2HtjdVMxJx3KgnEu3kSR");
      let metadataAccount = await Metadata.getPDA(MintAccount);
      const curr_metadata = await Metadata.load(connection, metadataAccount);
    
        let newMetadataData = new MetadataDataData({
          name: curr_metadata.data.data.name,
          symbol: curr_metadata.data.data.symbol,
          uri: newUri,
          creators: [...creators],
          sellerFeeBasisPoints: curr_metadata.data.data.sellerFeeBasisPoints,
        })
        const update = new UpdateMetadata(
          { feePayer: signer.publicKey },
          {
            metadata: metadataAccount,
            updateAuthority: signer.publicKey,
            metadataData: newMetadataData,
            newUpdateAuthority: signer.publicKey,
            primarySaleHappened: curr_metadata.data.primarySaleHappened,
          },
        );
        let result = await sendAndConfirmTransaction(conn, update, [signer]);
        console.log("result =", result);
      }
    
    //using a cron service to schedule updates
    //will start the update on 1st April
    const startdate = new Date(2022,3,1,0,0,0);
    //will update once again on 1st May
    const changedate = new Date(2022, 4, 1, 0, 0, 0);
    
    const changetoETH = scheduleJob(startdate, async function () {
      console.log('changed to ETH');
      await updateMetadata("https://www.arweave.net/a1a5wH8fMKml8g2zBnQIWv2VYTIbIAFL0MYpUERWsn8?ext=jpg")
    });
    
    const changetoSOL = scheduleJob(changedate, async function () {
      console.log('CHANGED TO SOL');
      await updateMetadata("https://www.arweave.net/fzta9yQGusDhFmv9EhZO4PcRMTbHySFYEo5_oXXKP20?ext=jpg")
    });


    
})();



