import type { Type2019 } from '../batch-10/file-2019.js';
export interface Type2020 {
  id: 2020;
  name: 'File2020';
  next: Type2019;
}

export function make2020(): Type2020 {
  return { id: 2020, name: 'File2020', next: null as unknown as Type2019 };
}
