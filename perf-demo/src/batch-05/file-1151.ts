import type { Type1150 } from '../batch-05/file-1150.js';
export interface Type1151 {
  id: 1151;
  name: 'File1151';
  next: Type1150;
}

export function make1151(): Type1151 {
  return { id: 1151, name: 'File1151', next: null as unknown as Type1150 };
}
