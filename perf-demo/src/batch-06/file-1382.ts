import type { Type1381 } from '../batch-06/file-1381.js';
export interface Type1382 {
  id: 1382;
  name: 'File1382';
  next: Type1381;
}

export function make1382(): Type1382 {
  return { id: 1382, name: 'File1382', next: null as unknown as Type1381 };
}
