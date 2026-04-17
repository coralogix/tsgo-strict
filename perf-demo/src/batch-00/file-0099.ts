import type { Type98 } from '../batch-00/file-0098.js';
export interface Type99 {
  id: 99;
  name: 'File99';
  next: Type98;
}

export function make99(): Type99 {
  return { id: 99, name: 'File99', next: null as unknown as Type98 };
}
