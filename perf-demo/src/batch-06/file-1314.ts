import type { Type1313 } from '../batch-06/file-1313.js';
export interface Type1314 {
  id: 1314;
  name: 'File1314';
  next: Type1313;
}

export function make1314(): Type1314 {
  return { id: 1314, name: 'File1314', next: null as unknown as Type1313 };
}
