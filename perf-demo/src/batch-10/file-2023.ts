import type { Type2022 } from '../batch-10/file-2022.js';
export interface Type2023 {
  id: 2023;
  name: 'File2023';
  next: Type2022;
}

export function make2023(): Type2023 {
  return { id: 2023, name: 'File2023', next: null as unknown as Type2022 };
}
