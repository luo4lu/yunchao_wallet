//创建钱包对象参数校验
import Joi from "joi";

export const createSchema = Joi.object({
    block:Joi.boolean().optional().default(false),
    encrypted_password:Joi.string().required(),
    reviewed:Joi.boolean().optional().default(false),
    channel:Joi.string().required(),
    extra:Joi.object().optional()
});

//锁定或解锁一个对象校验
export const blockSchema = Joi.object<{block:boolean}>({
    block:Joi.boolean().required()
});

