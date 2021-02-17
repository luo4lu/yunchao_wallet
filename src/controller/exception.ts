import {ExtendableContext, Next} from "koa";

const catchError = async (ctx:ExtendableContext, next:Next) => {
    try {
        await next()
    } catch (error) {
        console.log(error)
        let msg = `${error.stack}`;
        //参数校验失败
        if(msg.startsWith("ValidationError")) {
            ctx.body = {
                "code": "param_error",
                "message": error.message,
            }
            ctx.status = 400;
        }else{
            ctx.body = {
                "code": "internal_error",
                "message": error.message,
            }
            ctx.status = 402;
        }
    }
}
export default catchError;
