<?php

echo '
        <!-- GDPR Bar -->
        <div class="gdpr-bar" data-plugin-gdpr>
            <div class="gdpr-bar-description">
                <p class="text-color-light opacity-7 mb-0">Wir verwenden Cookies, um die Benutzerfreundlichkeit zu verbessern und den Website-Verkehr zu analysieren. Lesen Sie, wie wir Cookies verwenden und wie Sie sie kontrollieren können, indem Sie auf "Datenschutzeinstellungen" klicken.</p>
            </div>
            <div class="gdpr-bar-actions">
                <a href="#" class="gdpr-preferences-trigger text-decoration-none btn btn-danger btn-modern btn-px-4 py-3 font-weight-bold">Datenschutz-Einstellungen</a>
                <a href="#" class="gdpr-agree-trigger text-decoration-none btn btn-primary btn-modern btn-px-4 py-3 font-weight-bold">Ich stimme zu</a>
            </div>
        </div>

        <!-- GDPR Preferences Popup -->
        <div class="gdpr-preferences-popup">
            <div class="gdpr-preferences-popup-content position-relative">
                <a href="#" class="gdpr-close-popup text-color-grey text-color-hover-dark">
                    <i class="fas fa-times"></i>
                </a>
                <form class="gdpr-preferences-form">
                    <div class="gdpr-preferences-popup-content-body">
                        <h4 class="text-color-dark font-weight-bold mb-3">Datenschutz-Einstellungen</h4>
                        <p>Wenn Sie eine Website besuchen, kann diese Informationen über Ihren Browser speichern oder abrufen, normalerweise in Form von Cookies. Da wir Ihr Recht auf Privatsphäre respektieren, können Sie wählen, ob Sie die Datenerfassung durch bestimmte Arten von Diensten zulassen möchten. Wenn Sie diese Dienste nicht zulassen, kann dies jedoch Ihre Erfahrung beeinträchtigen.</p>
                        <hr>

                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">Datenschutzbestimmungen</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">Sie haben unsere Nutzungsbedingungen gelesen und akzeptiert <a href="privacy_policy.phtml">Datenschutzbestimmungen</a></p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">ERFORDERLICH</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="privacy-policy" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">CDN</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">Aus Gründen der Leistung und Sicherheit verwenden wir Cloudflare als unser CDN-Netzwerk.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">ERFORDERLICH</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="cdn" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">Kundenportal</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">Wir verwenden Sessions, um Ihr Kundenportal-Login zu verwalten.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">ERFORDERLICH</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="customer-portal" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">YouTube</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">Wir verwenden den YouTube-Dienst, um das Streaming von Videoinhalten auf dieser Website zu ermöglichen.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <input type="checkbox" name="cookies_consent[]" class="gdpr-input custom-checkbox-switch" value="youtube" checked/>
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">Vimeo</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">Wir verwenden den Dienst Vimeo, um das Streaming von Videoinhalten auf dieser Website zu ermöglichen.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <input type="checkbox" name="cookies_consent[]" class="gdpr-input custom-checkbox-switch" value="vimeo" checked/>
                            </div>
                        </div>
                    </div>
                    <div class="gdpr-preferences-popup-content-footer">
                        <a href="#" class="gdpr-reset-cookies btn btn-secondary btn-modern btn-px-4 py-3">Cookies zurücksetzen</a>
                        <button type="submit" class="btn btn-primary btn-modern btn-px-4 py-3 font-weight-bold">Präferenzen speichern</button>
                    </div>
                </form>
            </div>
        </div> 
';
?>
