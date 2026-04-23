import type { Type31 } from '../batch-00/file-0031.js';
export interface Type32 {
  id: 32;
  name: 'File32';
  next: Type31;
}

export function make32(): Type32 {
  return { id: 32, name: 'File32', next: null as unknown as Type31 };
}
