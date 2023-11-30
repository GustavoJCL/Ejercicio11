export interface TableData {
  length1: number;
  length2: number;
  total_length: number;
  ei: number;
  es: number;
  coeficient_capacity: number;
  coeficient_capacity_k: number;
  bar_state: boolean;
  probability: number;
}

export interface TableDataCollection {
  [key: number]: TableData;
}

export interface TableReplicas {
  [key: number]: TableDataCollection;
}

export interface DataReplicas {
  [key: number]: number[];
}
