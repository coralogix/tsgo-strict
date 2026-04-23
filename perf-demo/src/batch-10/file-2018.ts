import type { Type2017 } from '../batch-10/file-2017.js';
export interface Type2018 {
  id: 2018;
  name: 'File2018';
  next: Type2017;
}

export function make2018(): Type2018 {
  return { id: 2018, name: 'File2018', next: null as unknown as Type2017 };
}
