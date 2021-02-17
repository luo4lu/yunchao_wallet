import path from 'path';
import fs from 'fs';

const file = path.join(path.resolve(__dirname, '..'), 'config.json'); //文件路径，__dirname为当前运行js文件的目录
const result = fs.readFileSync(file, {encoding:'utf-8'});
export default JSON.parse(result);
