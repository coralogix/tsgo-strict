import type { Type2020 } from '../batch-10/file-2020.js';
export interface Type2021 {
  id: 2021;
  name: 'File2021';
  next: Type2020;
}

export function make2021(): Type2021 {
  return { id: 2021, name: 'File2021', next: null as unknown as Type2020 };
}
