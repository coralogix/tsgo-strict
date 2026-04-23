import type { Type1280 } from '../batch-06/file-1280.js';
export interface Type1281 {
  id: 1281;
  name: 'File1281';
  next: Type1280;
}

export function make1281(): Type1281 {
  return { id: 1281, name: 'File1281', next: null as unknown as Type1280 };
}
