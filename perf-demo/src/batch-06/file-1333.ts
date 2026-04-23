import type { Type1332 } from '../batch-06/file-1332.js';
export interface Type1333 {
  id: 1333;
  name: 'File1333';
  next: Type1332;
}

export function make1333(): Type1333 {
  return { id: 1333, name: 'File1333', next: null as unknown as Type1332 };
}
