import type { Type3101 } from '../batch-15/file-3101.js';
export interface Type3102 {
  id: 3102;
  name: 'File3102';
  next: Type3101;
}

export function make3102(): Type3102 {
  return { id: 3102, name: 'File3102', next: null as unknown as Type3101 };
}
