import type { Type2023 } from '../batch-10/file-2023.js';
export interface Type2024 {
  id: 2024;
  name: 'File2024';
  next: Type2023;
}

export function make2024(): Type2024 {
  return { id: 2024, name: 'File2024', next: null as unknown as Type2023 };
}
