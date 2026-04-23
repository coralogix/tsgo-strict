import type { Type2018 } from '../batch-10/file-2018.js';
export interface Type2019 {
  id: 2019;
  name: 'File2019';
  next: Type2018;
}

export function make2019(): Type2019 {
  return { id: 2019, name: 'File2019', next: null as unknown as Type2018 };
}
