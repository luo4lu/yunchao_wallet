import {Context, Next} from "koa";
import {createSchema} from "./schema/settle";
import {getManager} from "typeorm";
import {v4} from "uuid";
import moment from "moment";
import Settle from "../entity/settle";
import {pageSchema} from "./schema/page";
import logger from "../util/logger";
import Wallet from "../entity/wallet";

//创建一个结算对象
export async function createSettle(ctx:Context,next:Next){
    logger.info("createSettle");
    let {wallet_id} = ctx.params;
    let repository = getManager().getRepository(Wallet);
    let wallet =  await repository.findOne({id:wallet_id})
    if(!wallet){
        ctx.fail("wallet_not_exist","钱包不存在");
    }else {
        let params = await createSchema.validateAsync(ctx.request.body);
        let repository = getManager().getRepository(Settle);
        params.id = v4();
        params.created = moment().unix();
        params.type = 'settle';
        params.wallet_id = wallet_id;
        let newSettle = repository.create(params);
        let result = await repository.save(newSettle);
        ctx.success(result);
    }
    await next();
}
//查询计算对象
export async function getSettle(ctx:Context,next:Next){
    logger.info("getSettle");
    let {wallet_id,id} = ctx.params;
    let repository = getManager().getRepository(Settle);
    let settle = await repository.findOne({wallet_id,id});
    if(!settle){
        ctx.fail("settle_not_exist","结算对象不存在")
    }else {
        ctx.success(settle);
    }
    await next();
}

//返回一个结算对象的分页格式数组
export async function pageSettles(ctx:Context,next:Next){
    logger.info("pageSettles");
    let {wallet_id} = ctx.params;
    let {offset,limit,created_begin,created_end} = await pageSchema.validateAsync(ctx.request.query);
    let repository = getManager().getRepository(Settle);
    let queryBuilder = repository.createQueryBuilder("settle");
    queryBuilder.where("settle.wallet_id=:wallet_id");
    if(created_begin){
        queryBuilder.andWhere("settle.created>=:created_begin")
    }
    if(created_end){
        queryBuilder.andWhere("settle.created<=:created_end")
    }
    if(created_begin){
        queryBuilder.setParameter("created_begin",created_begin)
    }
    if(created_end){
        queryBuilder.setParameter("created_end",created_end)
    }
    queryBuilder.setParameter("wallet_id",wallet_id);
    let [items,total] = await queryBuilder.addOrderBy("settle.created","ASC").offset(offset).take(limit).getManyAndCount();
    ctx.success(items,total);
    await next();
}

export async function deleteSettle(ctx:Context,next:Next){
    logger.info("deleteSettle");
    let {wallet_id,id} = ctx.params;
    let repository = getManager().getRepository(Settle);
    let old = await repository.findOne({id,wallet_id});
    let result = await repository.delete({id,wallet_id});
    if(old && (result.affected||0)>0) {
        ctx.success(old);
    }else{
        ctx.fail("settle_not_exist","结算对象不存在");
    }
    await next();
}
