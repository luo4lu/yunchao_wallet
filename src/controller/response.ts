import {Context, Next} from "koa";

export function response(){
    return async function(ctx:Context,next:Next){
        ctx.success = function (data:any,total:number|undefined=undefined) {
            ctx.type = 'json'
            ctx.body = {
                code : 0,
                message : "success",
                data : data||null,
                total
            }
        }

        ctx.fail = function (code:string,msg:string) {
            ctx.type = 'json'
            ctx.body = {
                code : code,
                message : msg
            }
        }

        await next()
    }

}
