import {blockWallet, createWallet, infoWallet, pageWallets, reviewWallet} from "./controller/wallet";
import {createSettle, deleteSettle, getSettle, pageSettles} from "./controller/settle";

export default [
    {
        path: "/wallets",
        method: "post",
        action: createWallet
    },
    {
        path: "/wallets/:id/block",
        method: "put",
        action: blockWallet
    },
    {
        path: "/wallets/:id/review",
        method: "put",
        action: reviewWallet
    },
    {
        path: "/wallets/:id",
        method: "get",
        action: infoWallet
    },
    {
        path: "/wallets",
        method: "get",
        action: pageWallets
    },
    {
        path: "/wallets/:wallet_id/settles",
        method: "post",
        action: createSettle
    },
    {
        path: "/wallets/:wallet_id/settles/:id",
        method: "get",
        action: getSettle
    },
    {
        path: "/wallets/:wallet_id/settles",
        method: "get",
        action: pageSettles
    },
    {
        path: "/wallets/:wallet_id/settles/:id",
        method: "delete",
        action: deleteSettle
    }
]
