import { decryptLeaves } from "./fileCreation";
import { createMerkleTree } from "./fileCreation";
import fetch from "node-fetch";


async function getDataFromSDrive(spacePubKey: string): Promise<Buffer> {
    //https://shdw-drive.genesysgo.net/47Wym2BZ1XVQh6a2o3Szx23g6VWThy5WHfGSBbLfeMrJ/testleaves.bin
    const url = "https://shdw-drive.genesysgo.net/" + spacePubKey + "/testleaves.bin";
    const data = fetch(url).then(res => res.buffer());
    return data;
}

// (async () => {
//     let startTime = Date.now() / 1000;
//     const data = await getDataFromSDrive("47Wym2BZ1XVQh6a2o3Szx23g6VWThy5WHfGSBbLfeMrJ");
//     let endTime = Date.now() / 1000;
//     console.log(`Time to fetch data: ${endTime - startTime}s`);
//     const leaves = decryptLeaves("password", data);
//     const tree = createMerkleTree(leaves);
//     console.log("Root: ", tree.getRoot().toString("hex"));
//     endTime = Date.now() / 1000;
//     console.log(`Time total: ${endTime - startTime}s`)
// })();