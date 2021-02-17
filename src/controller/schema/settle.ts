import Joi from "joi";

//创建结算对象schema
export const createSchema = Joi.object({
    channel:Joi.string().required(),
    recipient:Joi.object({
        account:Joi.string().required(),
        name:Joi.string().required(),
        open_bank_code:Joi.string().required(),
        open_bank:Joi.string().optional(),
        card_type:Joi.number().min(0).max(4).optional(),
        sub_bank:Joi.string().max(80).min(1).optional(),
        sub_bank_code:Joi.string().optional(),
        prov:Joi.string().optional(),
        city:Joi.string().optional(),
    }).required(),
    reviewed:Joi.boolean().default(false),
    extra:Joi.object().optional()
});
