use calamine::{open_workbook, Error, RangeDeserializerBuilder, Reader, Xlsx};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Deserialize, Clone)]
struct PieceOriginal {
    #[serde(alias = "Title")]
    title: Option<String>,
    #[serde(alias = "Subtitle")]
    subtitle: Option<String>,
    #[serde(alias = "Composer")]
    composer: Option<String>,
    #[serde(alias = "Orig. composer")]
    original_composer: Option<String>,
    #[serde(alias = "Source")]
    source: Option<String>,
    #[serde(alias = "Document")]
    document: Option<String>,
    #[serde(alias = "Volume")]
    volume: Option<String>,
    #[serde(alias = "Date")]
    date: Option<String>,
    #[serde(alias = "Page")]
    page: Option<String>,
    #[serde(alias = "Editor")]
    editor: Option<String>,
    #[serde(alias = "Encoder")]
    encoder: Option<String>,
    #[serde(alias = "Arranger")]
    arranger: Option<String>,
    #[serde(alias = "Intabulator")]
    intabulator: Option<String>,
    #[serde(alias = "Contributor")]
    contributor: Option<String>,
    #[serde(alias = "Concordances")]
    concordances: Option<String>,
    #[serde(alias = "Piece")]
    piece: Option<String>,
    #[serde(alias = "Section")]
    section: Option<String>,
    #[serde(alias = "Type")]
    type_of_piece: Option<String>,
    #[serde(alias = "Key")]
    key: Option<String>,
    #[serde(alias = "Difficulty")]
    difficulty: Option<i8>,
    #[serde(alias = "Ensemble")]
    ensemble: Option<String>,
    #[serde(alias = "Part")]
    part: Option<String>,
    #[serde(alias = "Remarks")]
    remarks: Option<String>,
    #[serde(alias = "Recording")]
    recording: Option<String>,
    #[serde(alias = "Facsimile")]
    facsimile: Option<String>,
    #[serde(alias = "Fronimo")]
    fronimo: Option<String>,
    #[serde(alias = "PDF")]
    pdf: Option<String>,
    #[serde(alias = "Midi")]
    midi: Option<String>,
    #[serde(alias = "Created")]
    created: Option<String>,
    #[serde(alias = "Modified")]
    modified: Option<String>,
}

#[derive(Debug, Serialize)]
struct PieceCleaned {
    id: Uuid,
    title: Option<String>,
    subtitle: Option<String>,
    composer: Option<String>,
    original_composer: Option<String>,
    source: Option<String>,
    document: Option<String>,
    volume: Option<String>,
    date: Option<i32>,
    is_date_approximate: bool,
    page: Option<String>,
    editor: Option<String>,
    encoder: Option<String>,
    arranger: Option<String>,
    intabulator: Option<String>,
    contributor: Option<String>,
    concordances: Option<String>,
    piece: Option<String>,
    section: Option<String>,
    type_of_piece: Option<String>,
    key: Option<String>,
    difficulty: Option<i8>,
    ensemble: Option<String>,
    part: Option<String>,
    remarks: Option<String>,
    recording: Option<String>,
    facsimile: Option<String>,
    fronimo: Option<String>,
    pdf: Option<String>,
    midi: Option<String>,
    created: Option<String>,
    modified: Option<String>,
}

// All this ceremony just so we can attempt to convert the date to a number
impl From<PieceOriginal> for PieceCleaned {
    fn from(piece_original: PieceOriginal) -> Self {
        PieceCleaned {
            id: Uuid::new_v4(),
            title: piece_original.title,
            subtitle: piece_original.subtitle,
            composer: piece_original.composer,
            original_composer: piece_original.original_composer,
            source: piece_original.source,
            document: piece_original.document,
            volume: piece_original.volume,
            date: None,
            is_date_approximate: false,
            page: piece_original.page,
            editor: piece_original.editor,
            encoder: piece_original.encoder,
            arranger: piece_original.arranger,
            intabulator: piece_original.intabulator,
            contributor: piece_original.contributor,
            concordances: piece_original.concordances,
            piece: piece_original.piece,
            section: piece_original.section,
            type_of_piece: piece_original.type_of_piece,
            key: piece_original.key,
            difficulty: piece_original.difficulty,
            ensemble: piece_original.ensemble,
            part: piece_original.part,
            remarks: piece_original.remarks,
            recording: piece_original.recording,
            facsimile: piece_original.facsimile,
            fronimo: piece_original.fronimo,
            pdf: piece_original.pdf,
            midi: piece_original.midi,
            created: piece_original.created,
            modified: piece_original.modified,
        }
    }
}

fn main() -> anyhow::Result<()> {
    println!("Opening spreadsheet");
    let mut workbook: Xlsx<_> = open_workbook("spreadsheet.xlsx")?;
    let range = workbook
        .worksheet_range("dft")
        .ok_or(Error::Msg("Cannot find 'dft'"))??;

    let iter = RangeDeserializerBuilder::new().from_range(&range)?;

    let mut csv_writer = csv::Writer::from_path("gerbode.csv")?;

    for piece in iter {
        match piece {
            Ok(piece) => {
                let piece: PieceOriginal = piece;
                let mut piece_cleaned = PieceCleaned::from(piece.clone());
                // Attempt to parse the date, and leave None if unsuccessful
                if let Some(date) = piece.date {
                    let numeric_date: String = date.chars().filter(|c| c.is_numeric()).collect();

                    match numeric_date.parse::<i32>() {
                        Ok(parsed_date) => {
                            piece_cleaned.date = Some(parsed_date);
                            piece_cleaned.is_date_approximate = date.len() != numeric_date.len();
                        }
                        Err(e) => {
                            println!("Error parsing date {date}: {e}");
                        }
                    };
                }

                csv_writer.serialize(piece_cleaned)?;
            }
            Err(e) => {
                println!("Error parsing row {e}");
            }
        };
    }

    println!("Success!");

    csv_writer.flush()?;

    Ok(())
}
