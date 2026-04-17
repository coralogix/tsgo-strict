import type { Type3111 } from '../batch-15/file-3111.js';
export interface Type3112 {
  id: 3112;
  name: 'File3112';
  next: Type3111;
}

export function make3112(): Type3112 {
  return { id: 3112, name: 'File3112', next: null as unknown as Type3111 };
}
