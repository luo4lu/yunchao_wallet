import Joi from "joi";
import {Context, Next} from "koa";
import {getManager} from "typeorm";
import Wallet from "../entity/wallet";
import {v4} from 'uuid';
import moment from 'moment';
import {blockSchema, createSchema} from "./schema/wallet";
import {pageSchema} from "./schema/page";
import logger from "../util/logger";

//创建一个 `wallet` 对象
export async function createWallet(ctx:Context,next:Next){
    logger.info("createWallet");
    let params = await createSchema.validateAsync(ctx.request.body);
    let repository = getManager().getRepository(Wallet);
    params.id = v4();
    params.created = moment().unix();
    params.type = 'wallet';
    let newWallet = repository.create(params);
    let result = await repository.save(newWallet);
    ctx.success(result);
    await next();
}

//锁定或解锁一个对象
export async function blockWallet(ctx:Context,next:Next){
    logger.info("blockWallet");
    let {id} = ctx.params;
    let params = await blockSchema.validateAsync(ctx.request.body);
    let repository = getManager().getRepository(Wallet);
    let result = await repository.findOne({id});
    if(result){
        result.block = params.block;
        result = await repository.save(result);
        ctx.success(result);
    }else {
        ctx.fail("wallet_not_exist", "钱包不存在");
    }
    await next();
}
//将钱包通过审核
export async function reviewWallet(ctx:Context, next:Next){
    logger.info("reviewWallet");
    let {id} = ctx.params;
    let repository = getManager().getRepository(Wallet);
    let result = await repository.findOne({id})
    if(result){
        result.reviewed = true;
        result = await repository.save(result);
        ctx.success(result);
    }else{
        ctx.fail("wallet_not_exist", "钱包不存在");
    }
    await next();
}
export async function infoWallet(ctx:Context, next:Next){
    let {id} = ctx.params;
    let repository = getManager().getRepository(Wallet);
    let result = await repository.findOne({id})
    if(result){
        ctx.success(result);
    }else{
        ctx.fail("wallet_not_exist", "钱包不存在");
    }
    await next();
}

export async function pageWallets(ctx:Context, next:Next){
    logger.info("pageWallets")
    let {limit,offset,created_begin,created_end} = await pageSchema.validateAsync(ctx.request.query);
    let repository = getManager().getRepository(Wallet);
    let queryBuilder = await repository.createQueryBuilder("wallet");
    queryBuilder.where("1=1")
    if(created_begin){
        queryBuilder.andWhere("wallet.created>=:created_begin")
    }
    if(created_end){
        queryBuilder.andWhere("wallet.created<=:created_end")
    }
    if(created_begin){
        queryBuilder.setParameter("created_begin",created_begin)
    }
    if(created_end){
        queryBuilder.setParameter("created_end",created_end)
    }
    let [items,total] = await queryBuilder.addOrderBy("created","ASC").skip(offset).take(limit).getManyAndCount();
    ctx.success(items,total);
    await next();
}
// router.post("/wallets",async (ctx,next)=>{
//     let value = await createSchema.validateAsync(ctx.request.body);
//     ctx.response.body='eeee';
//     await next();
// })
//
//
// router.put("/wallets/:id/block",async (ctx,next)=>{
//     let {id} = ctx.params
// })
//
// router.put("/wallets/:id/review",async(ctx,next)=>{
//
// })
// router.get("/wallets/:id",async (ctx,next)=>{
//
// })
// router.get("/wallets",async (ctx,next)=>{
//     let wallet = await Repository.wallet;
//     // wallet.find({ta``});
// })
// // console.log(router);
// export default router;
