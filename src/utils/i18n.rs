use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    Auto,
    English,
    Portuguese,
    Spanish,
    German,
    French,
    Russian,
    Japanese,
}

impl Language {
    pub fn code(&self) -> &'static str {
        match self {
            Language::Auto       => "auto",
            Language::English    => "en",
            Language::Portuguese => "pt",
            Language::Spanish    => "es",
            Language::German     => "de",
            Language::French     => "fr",
            Language::Russian    => "ru",
            Language::Japanese   => "ja",
        }
    }

    pub fn from_code(code: &str) -> Self {
        match code.to_lowercase().as_str() {
            "en" | "english"               => Language::English,
            "pt" | "pt-br" | "pt-pt" | "portuguese" => Language::Portuguese,
            "es" | "spanish" | "espanol"   => Language::Spanish,
            "de" | "german" | "deutsch"    => Language::German,
            "fr" | "french" | "francais"   => Language::French,
            "ru" | "russian"               => Language::Russian,
            "ja" | "japanese"              => Language::Japanese,
            _                              => Language::Auto,
        }
    }

    pub fn display_info(&self) -> (&'static str, &'static str) {
        match self {
            Language::Auto       => ("Auto (System)", "Detect OS Language"),
            Language::English    => ("English (US)", "Default Global"),
            Language::Portuguese => ("Português (Brasil)", "Portuguese"),
            Language::Spanish    => ("Español", "Spanish"),
            Language::German     => ("Deutsch", "German"),
            Language::French     => ("Français", "French"),
            Language::Russian    => ("Русский", "Russian"),
            Language::Japanese   => ("日本語", "Japanese"),
        }
    }

    pub fn all_available() -> &'static [Language] {
        &[
            Language::Auto,
            Language::English,
            Language::Portuguese,
            Language::Spanish,
            Language::German,
            Language::French,
            Language::Russian,
            Language::Japanese,
        ]
    }
}

pub struct Translations {
    pub login_title: &'static str,
    pub login_desc: &'static str,
    pub login_placeholder: &'static str,
    pub login_btn_connect: &'static str,
    pub login_btn_detect: &'static str,
    pub server_channels: &'static str,
    pub leave: &'static str,
    pub view_text_chat: &'static str,
    pub leave_call: &'static str,
    pub voice_participants_title: &'static str,
    pub you: &'static str,
    pub view_voice_room: &'static str,
    pub replying_to: &'static str,
    pub chat_placeholder_prefix: &'static str,
    pub send: &'static str,
    pub settings_title: &'static str,
    pub settings_language_label: &'static str,
    pub settings_input_device: &'static str,
    pub settings_output_device: &'static str,
    pub settings_threshold: &'static str,
    pub settings_mic_level: &'static str,
    pub settings_done: &'static str,
    pub logout_title: &'static str,
    pub logout_confirm_prefix: &'static str,
    pub cancel: &'static str,
    pub confirm_logout: &'static str,
    pub voice_connecting: &'static str,
    pub voice_connecting_title: &'static str,
    pub voice_connecting_desc: &'static str,
    pub voice_connected: &'static str,
    pub qr_title: &'static str,
    pub qr_desc: &'static str,
    pub qr_confirm: &'static str,
    pub settings_tab_voice: &'static str,
    pub settings_tab_language: &'static str,
    pub settings_tab_keybinds: &'static str,
    pub settings_tab_security: &'static str,
    pub settings_tab_network_proxy: &'static str,
    pub settings_tab_updates: &'static str,
    pub keybind_mute_title: &'static str,
    pub keybind_mute_desc: &'static str,
    pub keybind_deafen_title: &'static str,
    pub keybind_deafen_desc: &'static str,
    pub keybind_recording_prompt: &'static str,
    pub keybind_btn_record: &'static str,
    pub keybind_btn_clear: &'static str,
    pub keybind_guide_title: &'static str,
    pub keybind_guide_desc: &'static str,
}

pub const EN_TRANSLATIONS: Translations = Translations {
    login_title: "Sign in to Kiyora",
    login_desc: "Enter your Discord User Token for direct connection to Gateway v9.\n(Estimated RAM usage ~15MB)",
    login_placeholder: "Paste your Discord User Token here...",
    login_btn_connect: "Connect to Gateway",
    login_btn_detect: "Detect Discord Token",
    server_channels: "SERVER CHANNELS",
    leave: "Leave",
    view_text_chat: "View Text Chat",
    leave_call: "Disconnect Call",
    voice_participants_title: "VOICE ROOM PARTICIPANTS",
    you: "YOU",
    view_voice_room: "View Voice Room",
    replying_to: "Replying to @",
    chat_placeholder_prefix: "Message ",
    send: "Send",
    settings_title: "Voice & Audio Settings",
    settings_language_label: "Interface Language",
    settings_input_device: "Input Device (Microphone)",
    settings_output_device: "Output Device (Speaker/Headset)",
    settings_threshold: "Activation Threshold (dBFS)",
    settings_mic_level: "Microphone Level",
    settings_done: "Done",
    logout_title: "Log Out",
    logout_confirm_prefix: "Are you sure you want to log out of ",
    cancel: "Cancel",
    confirm_logout: "Log Out",
    voice_connecting: "Connecting to voice...",
    voice_connecting_title: "Connecting to Voice",
    voice_connecting_desc: "Establishing encrypted voice connection...",
    voice_connected: "Voice Connected",
    qr_title: "Sign in with QR Code",
    qr_desc: "Scan with the Discord mobile app to sign in instantly.",
    qr_confirm: "Confirm sign-in on your phone!",
    settings_tab_voice: "Voice & Audio",
    settings_tab_language: "Language",
    settings_tab_keybinds: "Keybinds",
    settings_tab_security: "Security & E2EE",
    settings_tab_network_proxy: "Network & Proxy",
    settings_tab_updates: "Updates & About",
    keybind_mute_title: "Toggle Mic Mute",
    keybind_mute_desc: "Global shortcut to mute/unmute your microphone during a call.",
    keybind_deafen_title: "Toggle Speaker Mute",
    keybind_deafen_desc: "Global shortcut to mute all audio and your microphone.",
    keybind_recording_prompt: "Press a key combination...",
    keybind_btn_record: "Record Key",
    keybind_btn_clear: "Clear",
    keybind_guide_title: "Global Shortcuts",
    keybind_guide_desc: "Works in the background, even during games or when minimized.",
};

pub const PT_TRANSLATIONS: Translations = Translations {
    login_title: "Entrar no Kiyora",
    login_desc: "Insira seu Token de Usuário do Discord para conexão direta ao Gateway v9.\n(Uso estimado de RAM ~15MB)",
    login_placeholder: "Cole aqui seu Token de Usuário do Discord...",
    login_btn_connect: "Conectar ao Gateway",
    login_btn_detect: "Detectar Token do Discord",
    server_channels: "CANAIS DO SERVIDOR",
    leave: "Sair",
    view_text_chat: "Ver Chat de Texto",
    leave_call: "Desconectar Chamada",
    voice_participants_title: "PARTICIPANTES DA SALA DE VOZ",
    you: "VOCÊ",
    view_voice_room: "Ver Sala de Voz",
    replying_to: "Respondendo a @",
    chat_placeholder_prefix: "Mensagem ",
    send: "Enviar",
    settings_title: "Configurações de Voz e Áudio",
    settings_language_label: "Idioma da Interface",
    settings_input_device: "Dispositivo de Entrada (Microfone)",
    settings_output_device: "Dispositivo de Saída (Alto-falante/Fone)",
    settings_threshold: "Limiar de Ativação (dBFS)",
    settings_mic_level: "Nível do Microfone",
    settings_done: "Concluído",
    logout_title: "Sair da Conta",
    logout_confirm_prefix: "Tem certeza que deseja sair de ",
    cancel: "Cancelar",
    confirm_logout: "Sair",
    voice_connecting: "Conectando à voz...",
    voice_connecting_title: "Conectando à Voz",
    voice_connecting_desc: "Estabelecendo conexão de voz criptografada...",
    voice_connected: "Voz Conectada",
    qr_title: "Entrar com QR Code",
    qr_desc: "Escaneie com o app móvel do Discord para entrar instantaneamente.",
    qr_confirm: "Confirme o login no seu telefone!",
    settings_tab_voice: "Voz & Áudio",
    settings_tab_language: "Idioma",
    settings_tab_keybinds: "Atalhos de Teclado",
    settings_tab_security: "Segurança & E2EE",
    settings_tab_network_proxy: "Rede & Proxy",
    settings_tab_updates: "Atualizações & Sobre",
    keybind_mute_title: "Alternar Mudo do Microfone",
    keybind_mute_desc: "Atalho global para silenciar/ativar o microfone durante uma chamada.",
    keybind_deafen_title: "Alternar Mudo do Alto-falante",
    keybind_deafen_desc: "Atalho global para silenciar todos os áudios e o microfone.",
    keybind_recording_prompt: "Pressione uma combinação de teclas...",
    keybind_btn_record: "Gravar Tecla",
    keybind_btn_clear: "Limpar",
    keybind_guide_title: "Atalhos Globais",
    keybind_guide_desc: "Funciona em segundo plano, mesmo durante jogos ou minimizado.",
};

pub const ES_TRANSLATIONS: Translations = Translations {
    login_title: "Iniciar sesión en Kiyora",
    login_desc: "Ingresa tu Token de Usuario de Discord para conectarte directamente al Gateway v9.\n(Uso estimado de RAM ~15MB)",
    login_placeholder: "Pega aquí tu Token de Usuario de Discord...",
    login_btn_connect: "Conectar al Gateway",
    login_btn_detect: "Detectar Token de Discord",
    server_channels: "CANALES DEL SERVIDOR",
    leave: "Salir",
    view_text_chat: "Ver Chat de Texto",
    leave_call: "Desconectar Llamada",
    voice_participants_title: "PARTICIPANTES DE LA SALA DE VOZ",
    you: "TÚ",
    view_voice_room: "Ver Sala de Voz",
    replying_to: "Respondiendo a @",
    chat_placeholder_prefix: "Mensaje ",
    send: "Enviar",
    settings_title: "Configuración de Voz y Audio",
    settings_language_label: "Idioma de la Interfaz",
    settings_input_device: "Dispositivo de Entrada (Micrófono)",
    settings_output_device: "Dispositivo de Salida (Altavoz/Auriculares)",
    settings_threshold: "Umbral de Activación (dBFS)",
    settings_mic_level: "Nivel del Micrófono",
    settings_done: "Listo",
    logout_title: "Cerrar Sesión",
    logout_confirm_prefix: "¿Seguro que quieres cerrar sesión de ",
    cancel: "Cancelar",
    confirm_logout: "Cerrar Sesión",
    voice_connecting: "Conectando a voz...",
    voice_connecting_title: "Conectando a Voz",
    voice_connecting_desc: "Estableciendo conexión de voz cifrada...",
    voice_connected: "Voz Conectada",
    qr_title: "Iniciar Sesión con Código QR",
    qr_desc: "Escanea con la app de Discord para iniciar sesión al instante.",
    qr_confirm: "¡Confirma el inicio de sesión en tu teléfono!",
    settings_tab_voice: "Voz & Audio",
    settings_tab_language: "Idioma",
    settings_tab_keybinds: "Atajos de Teclado",
    settings_tab_security: "Seguridad & E2EE",
    settings_tab_network_proxy: "Red & Proxy",
    settings_tab_updates: "Actualizaciones & Acerca de",
    keybind_mute_title: "Alternar Silencio del Micrófono",
    keybind_mute_desc: "Atajo global para silenciar/activar el micrófono durante una llamada.",
    keybind_deafen_title: "Alternar Silencio del Altavoz",
    keybind_deafen_desc: "Atajo global para silenciar todo el audio y el micrófono.",
    keybind_recording_prompt: "Presiona una combinación de teclas...",
    keybind_btn_record: "Grabar Tecla",
    keybind_btn_clear: "Limpiar",
    keybind_guide_title: "Atajos Globales",
    keybind_guide_desc: "Funciona en segundo plano, incluso durante juegos o minimizado.",
};

pub const DE_TRANSLATIONS: Translations = Translations {
    login_title: "Bei Kiyora anmelden",
    login_desc: "Gib deinen Discord-Benutzer-Token ein, um direkt mit dem Gateway v9 zu verbinden.\n(Geschätzter RAM-Verbrauch ~15MB)",
    login_placeholder: "Füge hier deinen Discord-Benutzer-Token ein...",
    login_btn_connect: "Mit Gateway verbinden",
    login_btn_detect: "Discord-Token erkennen",
    server_channels: "SERVERKANÄLE",
    leave: "Verlassen",
    view_text_chat: "Textchat anzeigen",
    leave_call: "Anruf beenden",
    voice_participants_title: "TEILNEHMER IM SPRACHRAUM",
    you: "DU",
    view_voice_room: "Sprachraum anzeigen",
    replying_to: "Antwort an @",
    chat_placeholder_prefix: "Nachricht ",
    send: "Senden",
    settings_title: "Sprach- und Audioeinstellungen",
    settings_language_label: "Oberflächensprache",
    settings_input_device: "Eingabegerät (Mikrofon)",
    settings_output_device: "Ausgabegerät (Lautsprecher/Kopfhörer)",
    settings_threshold: "Aktivierungsschwelle (dBFS)",
    settings_mic_level: "Mikrofonpegel",
    settings_done: "Fertig",
    logout_title: "Abmelden",
    logout_confirm_prefix: "Bist du sicher, dass du dich abmelden möchtest von ",
    cancel: "Abbrechen",
    confirm_logout: "Abmelden",
    voice_connecting: "Verbinde mit Sprache...",
    voice_connecting_title: "Verbinde mit Sprache",
    voice_connecting_desc: "Verschlüsselte Sprachverbindung wird hergestellt...",
    voice_connected: "Sprachverbindung hergestellt",
    qr_title: "Mit QR-Code anmelden",
    qr_desc: "Scanne mit der Discord-App, um dich sofort anzumelden.",
    qr_confirm: "Bestätige die Anmeldung auf deinem Telefon!",
    settings_tab_voice: "Sprache & Audio",
    settings_tab_language: "Sprache",
    settings_tab_keybinds: "Tastenkürzel",
    settings_tab_security: "Sicherheit & E2EE",
    settings_tab_network_proxy: "Netzwerk & Proxy",
    settings_tab_updates: "Updates & Info",
    keybind_mute_title: "Mikrofon stumm schalten",
    keybind_mute_desc: "Globales Kürzel zum Stummschalten/Aktivieren des Mikrofons während eines Anrufs.",
    keybind_deafen_title: "Lautsprecher stumm schalten",
    keybind_deafen_desc: "Globales Kürzel zum Stummschalten aller Töne und des Mikrofons.",
    keybind_recording_prompt: "Tastenkombination drücken...",
    keybind_btn_record: "Taste aufnehmen",
    keybind_btn_clear: "Löschen",
    keybind_guide_title: "Globale Tastenkürzel",
    keybind_guide_desc: "Funktioniert im Hintergrund, auch während Spielen oder bei Minimierung.",
};

pub const FR_TRANSLATIONS: Translations = Translations {
    login_title: "Se connecter à Kiyora",
    login_desc: "Entrez votre Token utilisateur Discord pour une connexion directe au Gateway v9.\n(Utilisation RAM estimée ~15MB)",
    login_placeholder: "Collez votre Token utilisateur Discord ici...",
    login_btn_connect: "Se connecter au Gateway",
    login_btn_detect: "Détecter le Token Discord",
    server_channels: "CANAUX DU SERVEUR",
    leave: "Quitter",
    view_text_chat: "Voir le Chat Textuel",
    leave_call: "Raccrocher",
    voice_participants_title: "PARTICIPANTS DU SALON VOCAL",
    you: "VOUS",
    view_voice_room: "Voir le Salon Vocal",
    replying_to: "Répondre à @",
    chat_placeholder_prefix: "Message ",
    send: "Envoyer",
    settings_title: "Paramètres Voix & Audio",
    settings_language_label: "Langue de l'Interface",
    settings_input_device: "Périphérique d'Entrée (Microphone)",
    settings_output_device: "Périphérique de Sortie (Haut-parleur/Casque)",
    settings_threshold: "Seuil d'Activation (dBFS)",
    settings_mic_level: "Niveau du Microphone",
    settings_done: "Terminé",
    logout_title: "Se Déconnecter",
    logout_confirm_prefix: "Êtes-vous sûr de vouloir vous déconnecter de ",
    cancel: "Annuler",
    confirm_logout: "Se Déconnecter",
    voice_connecting: "Connexion vocale...",
    voice_connecting_title: "Connexion Vocale",
    voice_connecting_desc: "Établissement de la connexion vocale chiffrée...",
    voice_connected: "Voix Connectée",
    qr_title: "Se connecter avec QR Code",
    qr_desc: "Scannez avec l'app Discord pour vous connecter instantanément.",
    qr_confirm: "Confirmez la connexion sur votre téléphone !",
    settings_tab_voice: "Voix & Audio",
    settings_tab_language: "Langue",
    settings_tab_keybinds: "Raccourcis Clavier",
    settings_tab_security: "Sécurité & E2EE",
    settings_tab_network_proxy: "Réseau & Proxy",
    settings_tab_updates: "Mises à jour & À propos",
    keybind_mute_title: "Basculer Muet Micro",
    keybind_mute_desc: "Raccourci global pour couper/activer le microphone pendant un appel.",
    keybind_deafen_title: "Basculer Muet Haut-parleur",
    keybind_deafen_desc: "Raccourci global pour couper tous les sons et le microphone.",
    keybind_recording_prompt: "Appuyez sur une combinaison de touches...",
    keybind_btn_record: "Enregistrer la touche",
    keybind_btn_clear: "Effacer",
    keybind_guide_title: "Raccourcis Globaux",
    keybind_guide_desc: "Fonctionne en arrière-plan, même pendant les jeux ou réduit.",
};

pub const RU_TRANSLATIONS: Translations = Translations {
    login_title: "Войти в Kiyora",
    login_desc: "Введите токен пользователя Discord для прямого подключения к Gateway v9.\n(Примерное использование RAM ~15МБ)",
    login_placeholder: "Вставьте токен пользователя Discord...",
    login_btn_connect: "Подключиться к Gateway",
    login_btn_detect: "Определить токен Discord",
    server_channels: "КАНАЛЫ СЕРВЕРА",
    leave: "Выйти",
    view_text_chat: "Текстовый чат",
    leave_call: "Отключить звонок",
    voice_participants_title: "УЧАСТНИКИ ГОЛОСОВОЙ КОМНАТЫ",
    you: "ВЫ",
    view_voice_room: "Голосовая комната",
    replying_to: "Ответить @",
    chat_placeholder_prefix: "Сообщение ",
    send: "Отправить",
    settings_title: "Настройки голоса и аудио",
    settings_language_label: "Язык интерфейса",
    settings_input_device: "Устройство ввода (Микрофон)",
    settings_output_device: "Устройство вывода (Динамик/Наушники)",
    settings_threshold: "Порог активации (дБФС)",
    settings_mic_level: "Уровень микрофона",
    settings_done: "Готово",
    logout_title: "Выход из аккаунта",
    logout_confirm_prefix: "Вы уверены, что хотите выйти из ",
    cancel: "Отмена",
    confirm_logout: "Выйти",
    voice_connecting: "Подключение к голосу...",
    voice_connecting_title: "Подключение к голосу",
    voice_connecting_desc: "Установка зашифрованного голосового соединения...",
    voice_connected: "Голос подключён",
    qr_title: "Войти через QR-код",
    qr_desc: "Отсканируйте мобильным приложением Discord для мгновенного входа.",
    qr_confirm: "Подтвердите вход на вашем телефоне!",
    settings_tab_voice: "Голос и аудио",
    settings_tab_language: "Язык",
    settings_tab_keybinds: "Горячие клавиши",
    settings_tab_security: "Безопасность и E2EE",
    settings_tab_network_proxy: "Сеть и прокси",
    settings_tab_updates: "Обновления и о программе",
    keybind_mute_title: "Включить/выключить микрофон",
    keybind_mute_desc: "Глобальный ярлык для включения/отключения микрофона во время звонка.",
    keybind_deafen_title: "Включить/выключить динамики",
    keybind_deafen_desc: "Глобальный ярлык для отключения всего звука и микрофона.",
    keybind_recording_prompt: "Нажмите комбинацию клавиш...",
    keybind_btn_record: "Записать клавишу",
    keybind_btn_clear: "Очистить",
    keybind_guide_title: "Глобальные горячие клавиши",
    keybind_guide_desc: "Работает в фоне, даже в играх или при свёрнутом окне.",
};

pub const JA_TRANSLATIONS: Translations = Translations {
    login_title: "Kiyoraにサインイン",
    login_desc: "DiscordユーザートークンをGateway v9に直接接続するために入力してください。\n(推定RAM使用量 ~15MB)",
    login_placeholder: "DiscordユーザートークンをここにペーストしてくださいK.",
    login_btn_connect: "Gatewayに接続",
    login_btn_detect: "Discordトークンを検出",
    server_channels: "サーバーチャンネル",
    leave: "退出",
    view_text_chat: "テキストチャットを表示",
    leave_call: "通話を切断",
    voice_participants_title: "ボイスルーム参加者",
    you: "あなた",
    view_voice_room: "ボイスルームを表示",
    replying_to: "@に返信中: ",
    chat_placeholder_prefix: "メッセージ ",
    send: "送信",
    settings_title: "音声・オーディオ設定",
    settings_language_label: "インターフェース言語",
    settings_input_device: "入力デバイス（マイク）",
    settings_output_device: "出力デバイス（スピーカー/ヘッドセット）",
    settings_threshold: "起動しきい値 (dBFS)",
    settings_mic_level: "マイクレベル",
    settings_done: "完了",
    logout_title: "ログアウト",
    logout_confirm_prefix: "ログアウトしてもよろしいですか？ ",
    cancel: "キャンセル",
    confirm_logout: "ログアウト",
    voice_connecting: "音声に接続しています...",
    voice_connecting_title: "音声に接続中",
    voice_connecting_desc: "暗号化された音声接続を確立しています...",
    voice_connected: "音声接続完了",
    qr_title: "QRコードでサインイン",
    qr_desc: "DiscordモバイルアプリでスキャンしてすぐにログインできてKいます。",
    qr_confirm: "スマホでログインを承認してください！",
    settings_tab_voice: "音声 & オーディオ",
    settings_tab_language: "言語",
    settings_tab_keybinds: "キー割り当て",
    settings_tab_security: "セキュリティ & E2EE",
    settings_tab_network_proxy: "ネットワーク＆プロキシ",
    settings_tab_updates: "アップデート & 詳細",
    keybind_mute_title: "マイクミュート切替",
    keybind_mute_desc: "通話中にマイクのミュート/ミュート解除を切り替えるグローバルショートカット。",
    keybind_deafen_title: "スピーカーミュート切替",
    keybind_deafen_desc: "すべての音声とマイクを遮断するグローバルショートカット。",
    keybind_recording_prompt: "キーの組み合わせを押してください...",
    keybind_btn_record: "キーを記録",
    keybind_btn_clear: "無効化",
    keybind_guide_title: "グローバルショートカット",
    keybind_guide_desc: "ゲーム中や最小化時でもバックグラウンドで動作します。",
};

/// Returns translations for the given language (falls back to English).
pub fn get_translations(lang: Language) -> &'static Translations {
    match lang {
        Language::Portuguese => &PT_TRANSLATIONS,
        Language::Spanish    => &ES_TRANSLATIONS,
        Language::German     => &DE_TRANSLATIONS,
        Language::French     => &FR_TRANSLATIONS,
        Language::Russian    => &RU_TRANSLATIONS,
        Language::Japanese   => &JA_TRANSLATIONS,
        _                    => &EN_TRANSLATIONS,
    }
}

const LANGUAGE_CONFIG_FILE: &str = ".kiyora_language.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
pub struct LanguageConfig {
    pub selected_language: String,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            // Kiyora defaults to English
            selected_language: "en".to_string(),
        }
    }
}

pub fn load_persisted_language_config() -> Language {
    if let Ok(data) = std::fs::read_to_string(LANGUAGE_CONFIG_FILE) {
        if let Ok(cfg) = serde_json::from_str::<LanguageConfig>(&data) {
            return Language::from_code(&cfg.selected_language);
        }
    }
    // Default: English
    Language::English
}

pub fn save_persisted_language_config(lang: Language) {
    let cfg = LanguageConfig {
        selected_language: lang.code().to_string(),
    };
    if let Ok(json) = serde_json::to_string_pretty(&cfg) {
        let _ = std::fs::write(LANGUAGE_CONFIG_FILE, json);
    }
}

#[cfg(target_os = "windows")]
pub fn detect_os_language() -> Language {
    if let Ok(lang) = std::env::var("LANG").or_else(|_| std::env::var("LC_ALL")) {
        let lower = lang.to_lowercase();
        if lower.starts_with("pt") { return Language::Portuguese; }
        if lower.starts_with("es") { return Language::Spanish; }
        if lower.starts_with("de") { return Language::German; }
        if lower.starts_with("fr") { return Language::French; }
        if lower.starts_with("ru") { return Language::Russian; }
        if lower.starts_with("ja") { return Language::Japanese; }
    }
    unsafe {
        let lang_id = windows_sys::Win32::Globalization::GetUserDefaultUILanguage();
        let primary_lang = lang_id & 0x03FF;
        match primary_lang {
            0x0016 => Language::Portuguese, // LANG_PORTUGUESE
            0x000A => Language::Spanish,    // LANG_SPANISH
            0x0007 => Language::German,     // LANG_GERMAN
            0x000C => Language::French,     // LANG_FRENCH
            0x0019 => Language::Russian,    // LANG_RUSSIAN
            0x0011 => Language::Japanese,   // LANG_JAPANESE
            _      => Language::English,
        }
    }
}
