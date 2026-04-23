import type { Type1202 } from '../batch-06/file-1202.js';
export interface Type1203 {
  id: 1203;
  name: 'File1203';
  next: Type1202;
}

export function make1203(): Type1203 {
  return { id: 1203, name: 'File1203', next: null as unknown as Type1202 };
}
