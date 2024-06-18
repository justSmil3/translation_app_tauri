<?php

echo '
        <!-- GDPR Bar -->
        <div class="gdpr-bar" data-plugin-gdpr>
            <div class="gdpr-bar-description">
                <p class="text-color-light opacity-7 mb-0">We use cookies to improve user experience and analyze website traffic. Read about how we use cookies and how you can control them by clicking "Privacy Preferences".</p>
            </div>
            <div class="gdpr-bar-actions">
                <a href="#" class="gdpr-preferences-trigger text-decoration-none btn btn-danger btn-modern btn-px-4 py-3 font-weight-bold">Privacy Preferences</a>
                <a href="#" class="gdpr-agree-trigger text-decoration-none btn btn-primary btn-modern btn-px-4 py-3 font-weight-bold">I Agree</a>
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
                        <h4 class="text-color-dark font-weight-bold mb-3">Privacy Preferences</h4>
                        <p>When you visit any website, it may store or retrieve information through your browser, usually in the form of cookies. Since we respect your right to privacy, you can choose not to permit data collection from certain types of services. However, not allowing these services may impact your experience.</p>
                        <hr>

                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">Privacy Policy</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">You read and agreed to our <a href="privacy_policy.phtml">Privacy Policy</a></p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">REQUIRED</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="privacy-policy" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">CDN</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">For performance and security reasons, we use Cloudflare as our CDN networks.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">REQUIRED</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="cdn" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">Customer Portal</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">We use sessions to handle your customer portal login.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <strong class="text-color-dark text-2">REQUIRED</strong>
                                <input type="hidden" name="cookies_consent[]" class="gdpr-input" value="customer-portal" />
                            </div>
                        </div>
                        <div class="gdpr-cookie-item">
                            <div class="gdpr-cookie-item-title">
                                <strong class="text-color-dark">YouTube</strong>
                            </div>
                            <div class="gdpr-cookie-item-description">
                                <p class="mb-0">We use the YouTube service to enable video content streaming on this site.</p>
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
                                <p class="mb-0">We use the Vimeo service to enable video content streaming on this site.</p>
                            </div>
                            <div class="gdpr-cookie-item-action">
                                <input type="checkbox" name="cookies_consent[]" class="gdpr-input custom-checkbox-switch" value="vimeo" checked/>
                            </div>
                        </div>
                    </div>
                    <div class="gdpr-preferences-popup-content-footer">
                        <a href="#" class="gdpr-reset-cookies btn btn-secondary btn-modern btn-px-4 py-3">Reset Cookies</a>
                        <button type="submit" class="btn btn-primary btn-modern btn-px-4 py-3 font-weight-bold">Save Preferences</button>
                    </div>
                </form>
            </div>
        </div> 
';
?>
