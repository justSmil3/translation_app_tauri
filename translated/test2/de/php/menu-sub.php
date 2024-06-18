<?php

$caller = basename($_SERVER["PHP_SELF"]);

echo '
    <section class="section section-default mt-0 mb-0 border-0 bg-color-dark-scale-6">
        <div class="row text-center">
            <div class="col">
                <a class="text-color-light" href="account_dashboard.phtml"><button class="btn btn-sm btn-rounded ' . (str_starts_with($caller, "account_dashboard.phtml") ? 'btn-primary' : 'btn-light') . ' btn-modern text-3 mt-1">Dashboard</button></a>
                <a class="text-color-light" href="account_seats.phtml"><button class="btn btn-sm btn-rounded ' . (str_starts_with($caller, "account_seats.phtml") ? 'btn-primary' : 'btn-light') . ' btn-modern text-3 mt-1">Sitze</button></a>
                <div class="btn-group">
                    <button class="btn btn-sm btn-rounded ' . (str_starts_with($caller, "account_download_worldcreator") ? 'btn-primary' : 'btn-light') . ' btn-modern text-3 dropdown-toggle mt-1" type="button" id="dropDownWorldCreator" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                     World Creator
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropDownWorldCreator">
                      <a class="dropdown-item" href="account_download_worldcreator.phtml?category=Latest">LATEST</a>                      
                      <a class="dropdown-item" href="account_download_worldcreator.phtml?category=Legacy">LEGACY</a>
                    </div>
                </div>
                <div class="btn-group">
                    <button class="btn btn-sm btn-rounded ' . (str_starts_with($caller, "account_download_bridge") ? 'btn-primary' : 'btn-light') . ' btn-modern text-3 dropdown-toggle mt-1" type="button" id="dropDownBridge" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                     Bridge-Tools
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropDownBridge">
                      <a class="dropdown-item" href="account_download_bridge.phtml?category=Unity">UNITY-BRÜCKE</a>
                      <a class="dropdown-item" href="account_download_bridge.phtml?category=Unreal">UNREAL BRIDGE</a>
                      <a class="dropdown-item" href="account_download_bridge.phtml?category=Blender">BLENDER BRIDGE</a>
                      <a class="dropdown-item" href="account_download_bridge.phtml?category=Cinema">CINEMA 4D BRIDGE</a>
                      <a class="dropdown-item" href="account_download_bridge.phtml?category=Houdini">HOUDINI BRIDGE</a>
                    </div>
                </div>
                <div class="btn-group">
                    <button class="btn btn-sm btn-rounded ' . (str_starts_with($caller, "account_download_assets") ? 'btn-primary' : 'btn-light') . ' btn-modern text-3 dropdown-toggle mt-1" type="button" id="dropDownBridge" data-toggle="dropdown" aria-haspopup="true" aria-expanded="false">
                     Vermögenswerte
                    </button>
                    <div class="dropdown-menu" aria-labelledby="dropDownBridge">
                      <a class="dropdown-item" href="account_download_assets.phtml?category=Materials">MATERIALIEN</a>
                    </div>
                </div>                
                <a class="text-color-light" href="https://bitethebytes.freshdesk.com/support/tickets/new" target="_blank"><button class="btn btn-sm btn-light btn-rounded btn-modern text-3 mt-1">Unterstützung anfordern</button></a>
                <a class="text-color-light" href="account_featurerequest.phtml"><button class="btn btn-sm ' . (str_starts_with($caller, "account_featurerequest.phtml") ? 'btn-primary' : 'btn-light') . ' btn-modern btn-rounded text-3 mt-1">Feature Anfrage</button></a>
                <a class="text-color-light" href="account_bugreport.phtml"><button class="btn btn-sm ' . (str_starts_with($caller, "account_bugreport.phtml") ? 'btn-primary' : 'btn-light') . ' btn-modern btn-rounded text-3 mt-1">Fehlerbericht</button></a>                
                <a class="text-color-danger" href="logout.phtml"><button class="btn btn-sm btn-light btn-modern btn-rounded text-3 mt-1 text-color-danger">Abmelden</button></a>
            </div>
        </div>
    </section>
';

?>