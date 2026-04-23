import type { Type2015 } from '../batch-10/file-2015.js';
export interface Type2016 {
  id: 2016;
  name: 'File2016';
  next: Type2015;
}

export function make2016(): Type2016 {
  return { id: 2016, name: 'File2016', next: null as unknown as Type2015 };
}
