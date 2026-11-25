//! Pointer-following dictation indicator translations.
#[rustfmt::skip]
pub(super) const TRANSLATIONS: &[(&str, [&str; 6])] = &[
    ("Indicator follows mouse", ["Wskaźnik podąża za myszą", "指示器跟随鼠标", "インジケーターがマウスに追従", "Anzeige folgt der Maus", "El indicador sigue al ratón", "Індикатор слідує за мишею"]),
    ("Keep the dictation balloon beside the mouse pointer while recording and processing. Works with any dictation shortcut", ["Pokazuj dymek dyktowania przy wskaźniku myszy podczas nagrywania i przetwarzania. Działa z każdym skrótem dyktowania", "录音和处理时，让听写气泡保持在鼠标指针旁。适用于任何听写快捷键", "録音中と処理中に音声入力の吹き出しをマウスポインターのそばに表示します。どの音声入力ショートカットでも使えます", "Zeigt die Diktatblase während der Aufnahme und Verarbeitung neben dem Mauszeiger. Funktioniert mit jedem Diktierkürzel", "Mantiene el globo de dictado junto al puntero durante la grabación y el procesamiento. Funciona con cualquier atajo de dictado", "Показує бульбашку диктування біля вказівника миші під час запису й обробки. Працює з будь-якою комбінацією диктування"]),
];

#[rustfmt::skip]
pub(super) const FRENCH: &[(&str, &str)] = &[
    ("Indicator follows mouse", "L’indicateur suit la souris"),
    ("Keep the dictation balloon beside the mouse pointer while recording and processing. Works with any dictation shortcut", "Garde la bulle de dictée près du pointeur pendant l’enregistrement et le traitement. Fonctionne avec tous les raccourcis de dictée"),
];

#[rustfmt::skip]
pub(super) const PORTUGUESE: &[(&str, &str)] = &[
    ("Indicator follows mouse", "Indicador acompanha o mouse"),
    ("Keep the dictation balloon beside the mouse pointer while recording and processing. Works with any dictation shortcut", "Mantém o balão do ditado perto do ponteiro do mouse durante a gravação e o processamento. Funciona com qualquer atalho de ditado"),
];

#[rustfmt::skip]
pub(super) const RUSSIAN: &[(&str, &str)] = &[
    ("Indicator follows mouse", "Индикатор следует за мышью"),
    ("Keep the dictation balloon beside the mouse pointer while recording and processing. Works with any dictation shortcut", "Показывает пузырёк диктовки рядом с указателем мыши во время записи и обработки. Работает с любой комбинацией диктовки"),
];

#[rustfmt::skip]
pub(super) const HUNGARIAN: &[(&str, &str)] = &[
    ("Indicator follows mouse", "A jelző követi az egeret"),
    ("Keep the dictation balloon beside the mouse pointer while recording and processing. Works with any dictation shortcut", "A diktálás buborékját az egérmutató mellett tartja felvétel és feldolgozás közben. Bármely diktálási gyorsbillentyűvel működik"),
];
