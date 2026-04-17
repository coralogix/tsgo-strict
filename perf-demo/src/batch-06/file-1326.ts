import type { Type1325 } from '../batch-06/file-1325.js';
export interface Type1326 {
  id: 1326;
  name: 'File1326';
  next: Type1325;
}

export function make1326(): Type1326 {
  return { id: 1326, name: 'File1326', next: null as unknown as Type1325 };
}
