import type { Type2011 } from '../batch-10/file-2011.js';
export interface Type2012 {
  id: 2012;
  name: 'File2012';
  next: Type2011;
}

export function make2012(): Type2012 {
  return { id: 2012, name: 'File2012', next: null as unknown as Type2011 };
}
