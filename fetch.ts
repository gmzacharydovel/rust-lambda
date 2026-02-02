import { readFileSync } from "node:fs";


const config = JSON.parse(String(readFileSync("./request.json")));
const authorization = config.headers.Authorization;

console.log(authorization);

const start = performance.now();
const response = await fetch("https://71bs55wjyl.execute-api.ap-northeast-1.amazonaws.com/hello", {
  headers: {
    Authorization: authorization
  }
});

console.log(await response.text());

console.log((performance.now() - start) / 1000 + 's');
