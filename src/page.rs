use crate::row::*;
use crate::{database::*};
use log::debug;


use std::{sync::Arc};

// pub trait Page {
// // pub fn iter(&self) -> Rc<Iterator<Row>> {}
// fn get_rows(&self) -> Arc<Vec<Row>>;
// }

pub struct HeapPage {
    // page_id: HeapPageID,
    row_scheme: Arc<RowScheme>,
    rows: Arc<Vec<Row>>,
    header: Vec<u8>,
}

impl HeapPage {
    pub fn new(row_scheme: Arc<RowScheme>, bytes: Vec<u8>) -> HeapPage {
        // let table_id = page_id.table_id;
        // let row_scheme = Database::global().get_catalog().get_row_scheme(table_id);
        let mut header: Vec<u8> = Vec::new();
        let header_size = HeapPage::get_header_size(&row_scheme);
        debug!("header size: {} bytes", header_size);
        for b in bytes[0..header_size].into_iter() {
            header.push(*b);
        }
        debug!("header: {:x?}", header);

        let _allocated_rows_count = get_allocated(&header);

        // read rows
        let mut rows: Vec<Row> = Vec::new();
        // let reader = BufReader::new(header);
        let mut start = header_size;
        let mut end = start + row_scheme.get_size();
        for slot_id in 0..HeapPage::get_rows_count(&row_scheme) {
            // let row: Row = read_row(reader, &row_scheme);
            let row: Row = Row::new(Arc::clone(&row_scheme), &bytes[start..end]);

            if HeapPage::is_slot_used(&header, slot_id) {
                // debug!("push rows: {:?}, start: {}, end: {}", row, start, end);
                rows.push(row);
            }

            start = end;
            end += row_scheme.get_size();
        }

        display_rows(&rows);

        HeapPage {
            // page_id,
            row_scheme,
            header,
            rows: Arc::new(rows),
        }
    }

    // TODO: only return alocated rows
    pub fn get_rows(&self) -> Arc<Vec<Row>> {
        Arc::clone(&self.rows)
    }

    fn get_rows_count(row_scheme: &RowScheme) -> usize {
        // Database::global().get_buffer_pool().get_page_size() * 8 / (row_scheme.get_size() * 8 + 1)
        PAGE_SIZE * 8 / (row_scheme.get_size() * 8 + 1)
    }

    fn get_header_size(row_scheme: &RowScheme) -> usize {
        // debug!("rows count: {}", HeapPage::get_rows_count(&row_scheme));
        // debug!("rows scheme size: {}", row_scheme.get_size());
        (HeapPage::get_rows_count(&row_scheme) + 7) / 8
    }

    fn is_slot_used(header: &Vec<u8>, slot_id: usize) -> bool {
        let byte_index = slot_id / 8;
        let byte = header[byte_index];
        let bit_index = slot_id % 8;
        (byte & (1 << (7 - bit_index))) != 0
    }
}

// impl Page for HeapPage {
// fn get_rows(&self) -> Arc<Vec<Row>> {
// Arc::clone(&self.rows)
// }
// }

// pub struct HeapPageID {
// table_id: i32,
// page_index: i32,
// }

fn get_allocated(_header: &Vec<u8>) -> usize {
    0
}

// fn read_row(reader: BufReader<Vec<u8>>, row_scheme: &RowScheme) -> Row {

// }
