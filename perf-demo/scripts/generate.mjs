import fs from 'node:fs';
import path from 'node:path';

const root = process.cwd();
const srcDir = path.join(root, 'perf-demo', 'src');
const totalFiles = Number(process.argv[2] ?? '4000');
const batchSize = Number(process.argv[3] ?? '200');

fs.mkdirSync(srcDir, { recursive: true });

const indexExports = [];

for (let i = 0; i < totalFiles; i += 1) {
  const batch = Math.floor(i / batchSize);
  const batchDir = path.join(srcDir, `batch-${String(batch).padStart(2, '0')}`);
  fs.mkdirSync(batchDir, { recursive: true });

  const fileName = `file-${String(i).padStart(4, '0')}.ts`;
  const filePath = path.join(batchDir, fileName);

  const prevRef =
    i > 0
      ? `import type { Type${i - 1} } from '../batch-${String(Math.floor((i - 1) / batchSize)).padStart(2, '0')}/file-${String(i - 1).padStart(4, '0')}.js';\n`
      : '';

  const content = `${prevRef}export interface Type${i} {\n  id: ${i};\n  name: 'File${i}';\n  next: ${i > 0 ? `Type${i - 1}` : 'null'};\n}\n\nexport function make${i}(): Type${i} {\n  return { id: ${i}, name: 'File${i}', next: null as ${i > 0 ? `unknown as Type${i - 1}` : 'null'} };\n}\n`;

  fs.writeFileSync(filePath, content, 'utf8');
  indexExports.push(
    `export * from './batch-${String(batch).padStart(2, '0')}/${fileName.replace(/\.ts$/, '.js')}';`
  );
}

fs.writeFileSync(path.join(srcDir, 'index.ts'), `${indexExports.join('\n')}\n`, 'utf8');

console.log(`Generated ${totalFiles} TypeScript files in perf-demo/src`);
