import Joi from "joi";

export const pageSchema = Joi.object<{limit:number,offset:number,created:number|undefined,created_begin:number|undefined,created_end:number|undefined}>({
    limit:Joi.number().optional().default(5),//每页记录
    offset:Joi.number().optional().default(0),//偏移量,
    // created:Joi.number().optional(),
    created_begin:Joi.number().optional(),
    created_end:Joi.number().optional(),
})
