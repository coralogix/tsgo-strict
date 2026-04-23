import type { Type1011 } from '../batch-05/file-1011.js';
export interface Type1012 {
  id: 1012;
  name: 'File1012';
  next: Type1011;
}

export function make1012(): Type1012 {
  return { id: 1012, name: 'File1012', next: null as unknown as Type1011 };
}
