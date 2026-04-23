import type { Type2014 } from '../batch-10/file-2014.js';
export interface Type2015 {
  id: 2015;
  name: 'File2015';
  next: Type2014;
}

export function make2015(): Type2015 {
  return { id: 2015, name: 'File2015', next: null as unknown as Type2014 };
}
