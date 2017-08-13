subshader "pod_teemto2_cockpit_Material0" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 1.0 1.0 1.0;
	texture "texture/vehicles/PodRacer/pod_teemto2_engine_id1";
}

subshader "pod_teemto2_cockpit_Material1" "StandardMesh/Default"
{
	lighting true;
	lightingSpecular false;
	materialDiffuse 0.588235 0.588235 0.588235;
	transparent true;
	blendSrc sourceAlpha;
	blendDest one;
	alphaTestRef 0.7;
	texture "Mods\GCMOD\Movies\PodRacerCockpit";
}

