import type { Type2111 } from '../batch-10/file-2111.js';
export interface Type2112 {
  id: 2112;
  name: 'File2112';
  next: Type2111;
}

export function make2112(): Type2112 {
  return { id: 2112, name: 'File2112', next: null as unknown as Type2111 };
}
