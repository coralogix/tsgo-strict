import type { Type2021 } from '../batch-10/file-2021.js';
export interface Type2022 {
  id: 2022;
  name: 'File2022';
  next: Type2021;
}

export function make2022(): Type2022 {
  return { id: 2022, name: 'File2022', next: null as unknown as Type2021 };
}
