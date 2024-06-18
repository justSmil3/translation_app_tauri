<?php

echo '
<header id="header"
        class="header-transparent header-effect-shrink"
        data-plugin-options="{\'stickyEnabled\': true, \'stickyEffect\': \'shrink\', \'stickyEnableOnBoxed\': true, \'stickyEnableOnMobile\': true, \'stickyChangeLogo\': true, \'stickyStartAt\': 30, \'stickyHeaderContainerHeight\': 70}">
    <div class="header-body border-top-0 bg-color-dark-scale-7 box-shadow-none">
        <div class="header-container container-fluid px-lg-4">
            <div class="header-row">
                <div class="header-column">
                    <div class="header-row">
                        <div class="header-logo">
                            <img alt="World Creator"
                                 width="330"                                 
                                 data-sticky-width="330"                                 
                                 src="../../img/logos/applications/WorldCreator.webp">
                        </div>                        
                    </div>
                </div>
                <div class="header-column justify-content-end">
                    <div class="header-row">                        
                        <div class="header-nav header-nav-line header-nav-top-line header-nav-top-line-with-border order-2 order-lg-1">
                            <div class="header-nav-main header-nav-main-square header-nav-main-effect-2 header-nav-main-sub-effect-1">                            
                                <nav class="collapse">
                                    <ul class="nav nav-pills" id="mainNav">';
                                        include 'menu.php';
                              echo '</ul>
                                </nav>
                            </div>
                            <button class="btn header-btn-collapse-nav"
                                    data-toggle="collapse"
                                    data-target=".header-nav-main nav">
                                <i class="fas fa-bars"></i>
                            </button>
                        </div>
                    </div>
                </div>
            </div>
        </div>
    </div>
</header>';
?>