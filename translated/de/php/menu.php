<?php

$caller = basename($_SERVER["PHP_SELF"]);

if(\Portal\Sales::$Sale != \Portal\SaleType::None)
{
    echo '
        <li>
            <a class="text-3 text-danger text-color-hover-light dropdown-item appear-animation" data-appear-animation="rubberBand" data-appear-animation-delay="200" href="buy.phtml">' .\Portal\Sales::getSaleName() .'</a>
        </li>
    ';
}

    echo '         
        <li>
            <a class="text-3 font-weight-semibold text-color-white dropdown-item" href="index.phtml">HOME</a>
        </li>
        <li>
            <a class="text-3 font-weight-semibold text-color-white dropdown-item" href="features.phtml">FEATURES</a>
        </li>  
        <li>
            <a class="text-3 font-weight-semibold text-color-white dropdown-item" href="gallery.phtml">GALERIE</a>
        </li>  
        <li class="dropdown">
            <a class="text-3 font-weight-semibold text-color-white dropdown-item dropdown-toggle" href="#">LERNEN</a>
            <ul class="dropdown-menu">
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://bitethebytes.freshdesk.com/support/solutions/folders/44001240071" target="_blank">TUTORIALS</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://bitethebytes.freshdesk.com/support/solutions/44000815875" target="_blank">DOKUMENTATION</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://bitethebytes.freshdesk.com/en/support/solutions/44000596986" target="_blank">FAQ</a>
                </li>
            </ul>   
        </li>  
        <li class="dropdown">
            <a class="text-3 font-weight-semibold text-color-white dropdown-item dropdown-toggle" href="#">SOZIALES</a>
            <ul class="dropdown-menu">
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://discord.gg/worldcreator" target="_blank">DISCORD</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.instagram.com/worldcreator3d/" target="_blank">INSTAGRAM</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.youtube.com/channel/UClabqa6PHVjXzR2Y7s1MP0Q/" target="_blank">YOUTUBE</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://vimeo.com/user82114310/" target="_blank">VIMEO</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.twitch.tv/worldcreator3d/" target="_blank">TWITCH</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.facebook.com/worldcreator3d/" target="_blank">FACEBOOK</a>
                </li>                
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.artstation.com/worldcreator" target="_blank">ARTSTATION</a>
                </li>
            </ul>   
        </li>
        <li>
            <a class="text-3 font-weight-semibold text-color-white dropdown-item" href="buy.phtml">KAUFEN</a>
        </li>
        <li>
            <a class="text-4 font-weight-semibold text-color-white dropdown-item" href="news.phtml"><i class="fas fa-book-open" data-bs-toggle="tooltip" data-bs-animation="false" data-bs-placement="bottom" title="News"></i></a>
        </li>';


        if(((isset($_SESSION['worldcreator_manager_email']) && isset($_SESSION['worldcreator_manager_password'])) || (isset($_SESSION['worldcreator_user_email']) && isset($_SESSION['worldcreator_user_password'])) && isset($_SESSION['last_visit'])))
        {
            echo '
                <li class="dropdown">
                    <a class="text-4 font-weight-semibold text-color-white dropdown-item dropdown-toggle ' . (str_starts_with($caller, "account_") ? 'active' : '') . '" href="#"><i class="fas fa-user-check" data-bs-toggle="tooltip" data-bs-animation="false" data-bs-placement="bottom" title="ACCOUNT"></i></a>
                    <ul class="dropdown-menu">
                        <li>
                            <a class="dropdown-item" href="account_dashboard.phtml">Dashboard</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_seats.phtml">Sitze</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_worldcreator.phtml?category=Latest">World Creator - Letzte</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_worldcreator.phtml?category=Legacy">World Creator - Vermächtnis</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_bridge.phtml?category=Unity">Brücke der Einheit</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_bridge.phtml?category=Unreal">Unreal Brücke</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_bridge.phtml?category=Blender">Blender Brücke</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_bridge.phtml?category=Cinema">Cinema4D-Brücke</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_bridge.phtml?category=Houdini">Houdini Brücke</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_download_assets.phtml?category=Materials">Materialien</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="https://bitethebytes.freshdesk.com/support/tickets/new" target="_blank">Unterstützung anfordern</a>
                        </li>                    
                        <li>
                            <a class="dropdown-item" href="account_featurerequest.phtml">Feature Anfrage</a>
                        </li>
                        <li>
                            <a class="dropdown-item" href="account_bugreport.phtml">Fehlerbericht</a>
                        </li>
                        <li>
                            <a class="dropdown-item text-color-secondary" href="logout.phtml"><i class="fas fa-sign-out-alt pr-1"></i>AUSLOGGEN</a>
                        </li>
                    </ul>
                </li>';
        }

        if(!isset($_SESSION['worldcreator_manager_email']) && !isset($_SESSION['worldcreator_manager_password']) && !isset($_SESSION['worldcreator_user_email']) && !isset($_SESSION['worldcreator_user_password']))
        {
            echo '
                <li>
                    <a class="text-4 font-weight-semibold text-color-white dropdown-item ' . ($caller == "login.phtml" ? 'active' : '') . '" href="login.phtml"><i class="fas fa-user" data-bs-toggle="tooltip" data-bs-animation="false" data-bs-placement="bottom" title="Login"></i></a>
                </li>';
        }

        echo '
        <li class="dropdown">
            <a class="text-4 font-weight-semibold text-color-white dropdown-item dropdown-toggle" href="#"><i class="fas text-4 fa-globe-americas"></i></a>
            <ul class="dropdown-menu">
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.world-creator.com/en/' . $caller . '" target="_self"><img src="../img/blank.gif" class="flag flag-us" alt="English" />&nbsp ENGLISH</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.world-creator.com/ko/' . $caller . '" target="_self"><img src="../img/blank.gif" class="flag flag-kr" alt="한국어" />&nbsp 한국어</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.world-creator.com/zh/' . $caller . '" target="_self"><img src="../img/blank.gif" class="flag flag-cn" alt="中文" />&nbsp 中文</a>
                </li>
                <li>
                    <a class="text-1 font-weight-semibold text-color-white dropdown-item" href="https://www.world-creator.com/ja/' . $caller . '" target="_self"><img src="../img/blank.gif" class="flag flag-jp" alt="日本語" />&nbsp 日本語</a>
                </li>
            </ul>   
        </li>
';

?>