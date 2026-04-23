import type { Type1301 } from '../batch-06/file-1301.js';
export interface Type1302 {
  id: 1302;
  name: 'File1302';
  next: Type1301;
}

export function make1302(): Type1302 {
  return { id: 1302, name: 'File1302', next: null as unknown as Type1301 };
}
