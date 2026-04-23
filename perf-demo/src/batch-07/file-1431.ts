import type { Type1430 } from '../batch-07/file-1430.js';
export interface Type1431 {
  id: 1431;
  name: 'File1431';
  next: Type1430;
}

export function make1431(): Type1431 {
  return { id: 1431, name: 'File1431', next: null as unknown as Type1430 };
}
