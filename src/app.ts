import Koa from 'koa';
import Config from './util/config';
import catchError from './controller/exception';
import bodyParser from 'koa-bodyparser';
import 'reflect-metadata';
import {createConnection} from "typeorm";
import Router from "koa-router";
import Routes from './routes'
import {response} from "./controller/response";
createConnection(Config.db).then((connection)=>{
    const app = new Koa();
    const router = new Router();
    Routes.forEach(
        route => router[route.method](route.path, route.action));
    app.use(bodyParser())
    app.use(catchError);
    app.use(response())
    app.use(router.routes());
    app.listen(Config.serverPort,()=>{
        console.log(`start on port ${Config.serverPort}`)
    });
})

