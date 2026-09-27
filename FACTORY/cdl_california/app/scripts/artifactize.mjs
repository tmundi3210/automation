// Convert the single-file build into an Artifact page body (the publisher adds doctype/head/body).
import { readFileSync, writeFileSync } from 'node:fs';
const src = readFileSync('dist-single/index.html', 'utf8');
const title = (src.match(/<title>[\s\S]*?<\/title>/) || ['<title>CDL Workshop CA</title>'])[0];
const styles = [...src.matchAll(/<style[^>]*>[\s\S]*?<\/style>/g)].map((m) => m[0]).join('\n');
const scripts = [...src.matchAll(/<script[^>]*>[\s\S]*?<\/script>/g)].map((m) => m[0]).join('\n');
const body = (src.match(/<body[^>]*>([\s\S]*?)<\/body>/) || ['', ''])[1].replace(/<script[^>]*>[\s\S]*?<\/script>/g, '');
const out = `${title}\n<meta name="description" content="Learn and pass the California CDL General Knowledge and Combination Vehicles tests.">\n${styles}\n${body}\n${scripts}\n`;
writeFileSync('dist-single/artifact.html', out);
console.log('artifact.html', (out.length / 1024).toFixed(0) + ' KB');
